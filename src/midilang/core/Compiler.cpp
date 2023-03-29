#include "Compiler.h"


MIDILang::Compiler::Compiler(MIDILang::Program* program, std::string path, MIDILang::ErrorHandler* errors) {
    this->path = path;
    this->fileBytes = std::vector<unsigned char>();
    this->fileLength = 4; // Size Of End Track Event
    this->program = program;
    this->symbolTable = new SymbolTable();
    this->errors = errors;
}

void MIDILang::Compiler::compile() {
    headerChunk();
    trackChunk();
    writeFile();
}

void MIDILang::Compiler::headerChunk() {
    std::vector<unsigned char> header = {
        /*-----Header-Data----//-------Value-|-Description--------*/

        0x4d, 0x54, 0x68, 0x64, // MThd | ASCII Header Chunk Type
        0x00, 0x00, 0x00, 0x06, // 6 | 32 Bit Header Size
        0x00, 0x00, // 0 | 16 Bit File Format | Single Track
        0x00, 0x01, // 1 | Number Of Track Chunks
        0x00, // 0 | Divisions In Terms Of Ticks Per Quarter Note
        0x62, // 98 | 98 Ticks Per Quarter Note
    };

    this->fileBytes.insert(this->fileBytes.end(), header.begin(), header.end());
}

void MIDILang::Compiler::trackChunk() {
    std::vector<unsigned char> trackHeader = {
        /*-----Track-Data----//-------Value-|-Description--------*/

        0x4d, 0x54, 0x72, 0x6b, // MTrk | ASCII Track Chunk Type
        0x00, 0x00, 0x00, 0x00, // To Be Overwritten Later | Track Length
    };

    std::vector<unsigned char> endOfTrack = {
        0x00, 0xff, 0x2f, 0x00, // End Of Track Event
    };

    this->fileBytes.insert(this->fileBytes.end(), trackHeader.begin(), trackHeader.end());
    compileFile();
    this->fileBytes.insert(this->fileBytes.end(), endOfTrack.begin(), endOfTrack.end());
    cleanUp();
}

void MIDILang::Compiler::compileFile() {
    this->program->accept(*this);
}

void MIDILang::Compiler::cleanUp() {
    /* Overwriting The Track Length Bytes */ {
        int lengthOffset = 18;
        int trackLength = this->fileLength;

        unsigned char tlb[4] = {
            static_cast<unsigned char>((trackLength & 0x000000ff) >> 0),
            static_cast<unsigned char>((trackLength & 0x0000ff00) >> 8),
            static_cast<unsigned char>((trackLength & 0x00ff0000) >> 16),
            static_cast<unsigned char>((trackLength & 0xff000000) >> 24),
        };

        this->fileBytes[lengthOffset + 0] = tlb[3];
        this->fileBytes[lengthOffset + 1] = tlb[2];
        this->fileBytes[lengthOffset + 2] = tlb[1];
        this->fileBytes[lengthOffset + 3] = tlb[0];
    }
}

void MIDILang::Compiler::writeFile() {
    std::ofstream file;
    file.open(this->path + ".mid", std::ios::out | std::ios::binary);
    file.write((char*) &this->fileBytes[0], this->fileBytes.size());
}

std::any MIDILang::Compiler::visitProgramNode(MIDILang::Program* node) {
    node->getStatements()->accept(*this);
    return nullptr;
}

std::any MIDILang::Compiler::visitStatementNode(MIDILang::Statement* node) {
    for (Declaration* decl: *node->getDeclarationStatements()) {
        decl->accept(*this);
    }

    for (Action* act: *node->getActionStatements()) {
        act->accept(*this);
    }

    return nullptr;
}

std::any MIDILang::Compiler::visitTuneDeclaration(MIDILang::Tune* declaration) {
    addTune(declaration->getName(), *declaration);
    return nullptr;
}

std::any MIDILang::Compiler::visitVariableDeclaration(VariableDecl* declaration) {
    int value = std::any_cast<int>(declaration->getValue()->accept(*this));
    addVariable(declaration->getName(), value);
    return nullptr;
}

std::any MIDILang::Compiler::visitAssignmentAction(Assignment* action) {
    int value = std::any_cast<int>(action->getExpr()->accept(*this));
    int scope = getVariableScope(action->getName());

    reassignVariable(action->getName(), value, scope);
    return nullptr;
}

std::any MIDILang::Compiler::visitLoopAction(Loop* action) {
    int iterations = std::any_cast<int>(action->getIterations()->accept(*this));

    for (int i = 0; i < iterations; i++) {
        this->symbolTable->incrementScope();
        action->getStatements()->accept(*this);

        // Prevents The Error Message From Being Repeated
        this->errors->displayIfHasErrors();
        this->symbolTable->dropAndDecrement();
    }

    return nullptr;
}

std::any MIDILang::Compiler::visitPlayAction(Play* action) {
    Tune tune = getTune(action->getTune());
    if (tune.getStatement() == nullptr) return nullptr;

    if (tune.getParameters().size() != action->getArguments().size()) {
        newError("Tune " + tune.getName().literal + " Expected " + std::to_string(static_cast<int>(tune.getParameters().size())) + " Argument(s) But Received " + std::to_string(static_cast<int>(action->getArguments().size())), tune.getName().line);
        return nullptr;
    }

    this->symbolTable->incrementScope();
    if (!tune.getParameters().empty()) {
        for (int i = 0; i < tune.getParameters().size(); i++) {
            Expression* expr = action->getArguments()[i];
            int value = std::any_cast<int>(expr->accept(*this));
            addVariable(tune.getParameters()[i], value);
        }
    }

    tune.getStatement()->accept(*this);
    this->symbolTable->dropAndDecrement();

    return nullptr;
}

std::any MIDILang::Compiler::visitNoteAction(Note* action) {
    int note = std::any_cast<int>(action->getNote()->accept(*this));
    int duration = std::any_cast<int>(action->getDuration()->accept(*this));
    int velocity = std::any_cast<int>(action->getVelocity()->accept(*this));

    if (note > 127) {
        newError("Note Value Cannot Be More Than 127", action->getLine());
    } if (velocity > 127) {
        newError("Velocity Value Cannot Be More Than 127", action->getLine());
    }

    std::vector<unsigned char> durationU8 = intToVLE(duration, action->getLine());
    int durationLength = durationU8.size();

    std::vector<unsigned char> trackEvent;
    std::vector<unsigned char> noteOn = {
        /*----Event-Data---//----Value-|-Description-----*/

        0x00, // 0 | Elapsed Time From The Previous Event
        0x90, // 9_0 | Note On Event
        (unsigned char) note, // Note To Be Played
        (unsigned char) velocity, // Velocity To Be Played At
    };
    std::vector<unsigned char> noteOff = {
        // Duration To Be Inserted Here
        0x80, // 8_0 | Note Off Event
        (unsigned char) note, // Note To Be Turned Off
        0x00, // 0 | Velocity
    };

    trackEvent.insert(trackEvent.end(), noteOn.begin(), noteOn.end());
    trackEvent.insert(trackEvent.end(), durationU8.begin(), durationU8.end());
    trackEvent.insert(trackEvent.end(), noteOff.begin(), noteOff.end());

    this->fileLength += 7 + durationLength;
    this->fileBytes.insert(this->fileBytes.end(), trackEvent.begin(), trackEvent.end());
    return nullptr;
}

std::any MIDILang::Compiler::visitBinaryExpression(Binary* expression) {
    int lvalue = std::any_cast<int>(expression->getLValue()->accept(*this));
    int rvalue = std::any_cast<int>(expression->getRValue()->accept(*this));

    switch (expression->getOpType()[0]) {
        case '+': return lvalue + rvalue;
        case '-': return lvalue - rvalue;
        case '*': return lvalue * rvalue;
        case '/': return lvalue / rvalue;
        default: return 0;
    }
}

std::any MIDILang::Compiler::visitLiteralExpression(Literal* expression) {
    return std::stoi(expression->getValue().literal);
}

std::any MIDILang::Compiler::visitVariableExpression(VariableExpr* expression) {
    return getVariable(expression->getName());
}

std::vector<unsigned char> MIDILang::Compiler::intToVLE(int duration, int line) {
    std::vector<unsigned char> subResult;

    if (duration > ((int) 0x0fffffff)) {
        newError("Duration Value Cannot Be More Than 268435455", line);
        subResult.push_back(0);
        return subResult;
    }

    if (duration <= 127) {
        subResult.push_back((unsigned char) duration);
        return subResult;
    }

    std::vector<unsigned char> result = std::vector<unsigned char>(4);

    for (int i = 3; i >= 0; i--) {
        result[i] = (unsigned char) (duration & 0x7f);

        if (i < 3) {
            result[i] = (unsigned char) (result[i] | 0x80);
        }

        duration >>= 7;

        if (duration < 1) {
            break;
        }
    }

    int index = 0;

    for (int i = 0; i < result.size(); i++) {
        if (result[i] != 0) {
            index = i;
            break;
        }
    }

    std::vector<unsigned char> finalResult(result.begin() + index, result.begin() + 4);
    return finalResult;
}

void MIDILang::Compiler::addTune(MIDILang::Token identifier, MIDILang::Tune tuneStmt) {
    bool result = this->symbolTable->addTune(identifier, tuneStmt);
    if (!result) newError("Tune " + identifier.literal + " Already Exists In This Scope", identifier.line);
}

MIDILang::Tune MIDILang::Compiler::getTune(MIDILang::Token identifier) {
    if (!this->symbolTable->hasTune(identifier)) {
        newError("Tune " + identifier.literal + " Does Not Exist In This Scope", identifier.line);
        return { identifier, std::vector<Token>(), nullptr };
    }

    return this->symbolTable->getTune(identifier);
}

void MIDILang::Compiler::addVariable(MIDILang::Token identifier, int value) {
    int result = this->symbolTable->addVariable(identifier, value);
    if (!result) newError("Variable " + identifier.literal + " Already Exists In This Scope", identifier.line);
}

int MIDILang::Compiler::getVariable(MIDILang::Token identifier) {
    if (!this->symbolTable->hasVariable(identifier)) {
        newError("Variable " + identifier.literal + " Does Not Exist In This Scope", identifier.line);
        return -1;
    }

    return this->symbolTable->getVariable(identifier);
}

int MIDILang::Compiler::getVariableScope(MIDILang::Token identifier) {
    if (!this->symbolTable->hasVariable(identifier)) {
        newError("Variable " + identifier.literal + " Does Not Exist In This Scope", identifier.line);
        return -1;
    }

    return this->symbolTable->getVariableScope(identifier);
}

void MIDILang::Compiler::reassignVariable(MIDILang::Token identifier, int value, int scope) {
    bool result = this->symbolTable->reassignVariable(identifier, value, scope);
    if (!result) newError("Variable " + identifier.literal + " Does Not Exist In This Scope", identifier.line);
}

void MIDILang::Compiler::newError(std::string message, int line) {
    this->errors->addError("Compiler Error", message, line);
}

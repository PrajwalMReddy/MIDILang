#include "Tune.h"


MIDILang::Tune::Tune(MIDILang::Token name, std::vector<Token> parameters, MIDILang::Statement* statement) {
    this->name = name;
    this->parameters = parameters;
    this->statement = statement;
}

std::any MIDILang::Tune::accept(MIDILang::DeclVisitor& visitor) {
    return visitor.visitTuneDeclaration(this);
}

MIDILang::Token MIDILang::Tune::getName() {
    return this->name;
}

std::vector<MIDILang::Token> MIDILang::Tune::getParameters() {
    return this->parameters;
}

MIDILang::Statement* MIDILang::Tune::getStatement() {
    return this->statement;
}

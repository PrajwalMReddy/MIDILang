#include "Program.h"


MIDILang::Program::Program(Statement* statements) {
    this->statements = statements;
}

std::any MIDILang::Program::accept(MIDILang::OtherVisitor& visitor) {
    return visitor.visitProgramNode(this);
}

MIDILang::Statement* MIDILang::Program::getStatements() {
    return this->statements;
}

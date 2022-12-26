#include "Loop.h"


MIDILang::Loop::Loop(MIDILang::Expression* iterations, MIDILang::Statement* statements) {
    this->iterations = iterations;
    this->statements = statements;
}

std::any MIDILang::Loop::accept(MIDILang::ActVisitor& visitor) {
    return visitor.visitLoopAction(this);
}

MIDILang::Expression* MIDILang::Loop::getIterations() {
    return this->iterations;
}

MIDILang::Statement* MIDILang::Loop::getStatements() {
    return this->statements;
}

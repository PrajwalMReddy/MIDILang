#include "While.h"


MIDILang::While::While(MIDILang::Expression* iterations, MIDILang::Statement* statements) {
    this->condition = iterations;
    this->statements = statements;
}

std::any MIDILang::While::accept(MIDILang::ActVisitor& visitor) {
    return visitor.visitWhileAction(this);
}

MIDILang::Expression* MIDILang::While::getCondition() {
    return this->condition;
}

MIDILang::Statement* MIDILang::While::getStatements() {
    return this->statements;
}

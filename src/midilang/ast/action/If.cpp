#include "If.h"


MIDILang::If::If(Expression* condition, Statement* ifStatements, Statement* elseStatements) {
    this->condition = condition;
    this->ifStatements = ifStatements;
    this->elseStatements = elseStatements;
}

std::any MIDILang::If::accept(MIDILang::ActVisitor& visitor) {
    return visitor.visitIfAction(this);
}

MIDILang::Expression* MIDILang::If::getCondition() {
    return this->condition;
}

MIDILang::Statement* MIDILang::If::getIfStatements() {
    return this->ifStatements;
}

MIDILang::Statement* MIDILang::If::getElseStatements() {
    return this->elseStatements;
}

bool MIDILang::If::hasElseCondition() {
    return this->elseStatements != nullptr;
}

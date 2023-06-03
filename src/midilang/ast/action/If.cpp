#include "If.h"


MIDILang::If::If(Expression* condition, Statement* ifStatements, Statement* elseStatements, If* nestedIfStatements) {
    this->condition = condition;
    this->ifStatements = ifStatements;
    this->elseStatements = elseStatements;
    this->nestedIfStatements = nestedIfStatements;
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

MIDILang::If* MIDILang::If::getNestedIfStatements() {
    return this->nestedIfStatements;
}

bool MIDILang::If::hasElseCondition() {
    return this->elseStatements != nullptr || this->nestedIfStatements != nullptr;
}

bool MIDILang::If::hasNestedIf() {
    return this->nestedIfStatements != nullptr;
}

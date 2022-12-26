#include "VariableExpr.h"


MIDILang::VariableExpr::VariableExpr(MIDILang::Token name) {
    this->name = name;
}

std::any MIDILang::VariableExpr::accept(MIDILang::ExprVisitor& visitor) {
    return visitor.visitVariableExpression(this);
}

MIDILang::Token MIDILang::VariableExpr::getName() {
    return this->name;
}

#include "Unary.h"


MIDILang::Unary::Unary(std::string opType, MIDILang::Expression* value) {
    this->opType = opType;
    this->value = value;
}

std::any MIDILang::Unary::accept(MIDILang::ExprVisitor& visitor) {
    return visitor.visitUnaryExpression(this);
}

std::string MIDILang::Unary::getOpType() {
    return this->opType;
}

MIDILang::Expression* MIDILang::Unary::getValue() {
    return this->value;
}

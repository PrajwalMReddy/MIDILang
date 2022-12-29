#include "Binary.h"


MIDILang::Binary::Binary(MIDILang::Expression* lvalue, std::string opType, MIDILang::Expression* rvalue): lvalue(lvalue), rvalue(rvalue) {
    this->lvalue = lvalue;
    this->opType = opType;
    this->rvalue = rvalue;
}

std::any MIDILang::Binary::accept(MIDILang::ExprVisitor& visitor) {
    return visitor.visitBinaryExpression(this);
}

MIDILang::Expression* MIDILang::Binary::getLValue() {
    return this->lvalue;
}

std::string MIDILang::Binary::getOpType() {
    return this->opType;
}

MIDILang::Expression* MIDILang::Binary::getRValue() {
    return this->rvalue;
}

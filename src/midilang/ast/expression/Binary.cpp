#include "Binary.h"


MIDILang::Binary::Binary(MIDILang::Literal lvalue, std::string opType, MIDILang::Literal rvalue): lvalue(lvalue), rvalue(rvalue) {
    this->lvalue = lvalue;
    this->opType = opType;
    this->rvalue = rvalue;
}

std::any MIDILang::Binary::accept(MIDILang::ExprVisitor& visitor) {
    return visitor.visitBinaryExpression(this);
}

MIDILang::Literal MIDILang::Binary::getLValue() {
    return this->lvalue;
}

std::string MIDILang::Binary::getOpType() {
    return this->opType;
}

MIDILang::Literal MIDILang::Binary::getRValue() {
    return this->rvalue;
}

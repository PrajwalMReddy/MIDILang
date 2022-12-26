#include "Literal.h"


MIDILang::Literal::Literal(MIDILang::Token value) {
    this->value = value;
}

std::any MIDILang::Literal::accept(MIDILang::ExprVisitor& visitor) {
    return visitor.visitLiteralExpression(this);
}

MIDILang::Token MIDILang::Literal::getValue() {
    return this->value;
}

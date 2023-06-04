#include "Grouping.h"


MIDILang::Grouping::Grouping(MIDILang::Expression* value) {
    this->value = value;
}

std::any MIDILang::Grouping::accept(MIDILang::ExprVisitor& visitor) {
    return visitor.visitGroupingExpression(this);
}

MIDILang::Expression* MIDILang::Grouping::getValue() {
    return this->value;
}

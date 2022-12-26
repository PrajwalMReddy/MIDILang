#include "Assignment.h"


MIDILang::Assignment::Assignment(MIDILang::Token name, MIDILang::Expression* expr) {
    this->name = name;
    this->expr = expr;
}

std::any MIDILang::Assignment::accept(MIDILang::ActVisitor& visitor) {
    return visitor.visitAssignmentAction(this);
}

MIDILang::Token MIDILang::Assignment::getName() {
    return this->name;
}

MIDILang::Expression* MIDILang::Assignment::getExpr() {
    return this->expr;
}

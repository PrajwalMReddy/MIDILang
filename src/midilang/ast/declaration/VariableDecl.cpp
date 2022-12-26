#include "VariableDecl.h"


MIDILang::VariableDecl::VariableDecl(MIDILang::Token name, MIDILang::Expression* value) {
    this->name = name;
    this->value = value;
}

std::any MIDILang::VariableDecl::accept(MIDILang::DeclVisitor& visitor) {
    return visitor.visitVariableDeclaration(this);
}

MIDILang::Token MIDILang::VariableDecl::getName() {
    return this->name;
}

MIDILang::Expression* MIDILang::VariableDecl::getValue() {
    return this->value;
}

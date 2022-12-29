#include "VariableVar.h"


MIDILang::VariableVar::VariableVar(int value, int scope) {
    this->value = value;
    this->scope = scope;
}

int MIDILang::VariableVar::getValue() {
    return this->value;
}

int MIDILang::VariableVar::getScope() {
    return this->scope;
}

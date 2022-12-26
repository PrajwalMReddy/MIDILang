#include "SymbolTable.h"


MIDILang::SymbolTable::SymbolTable() {
    this->tunes = std::unordered_map<std::string, Tune>();
    this->variables = std::unordered_map<std::string, int>();
}

bool MIDILang::SymbolTable::addTune(MIDILang::Token identifier, MIDILang::Tune tuneStmt) {
    if (this->tunes.find(identifier.literal) != this->tunes.end()) return false;

    this->tunes.insert({ identifier.literal, tuneStmt });
    return true;
}

MIDILang::Tune MIDILang::SymbolTable::getTune(MIDILang::Token identifier) {
    return this->tunes.at(identifier.literal);
}

void MIDILang::SymbolTable::dropTune(MIDILang::Token identifier) {
    this->tunes.erase(identifier.literal);
}

bool MIDILang::SymbolTable::hasTune(MIDILang::Token identifier) {
    if (this->tunes.find(identifier.literal) != this->tunes.end()) return true;
    else return false;
}

bool MIDILang::SymbolTable::addVariable(MIDILang::Token identifier, int value) {
    if (this->variables.find(identifier.literal) != this->variables.end()) return false;

    this->variables.insert({ identifier.literal, value });
    return true;
}

int MIDILang::SymbolTable::getVariable(MIDILang::Token identifier) {
    return this->variables.at(identifier.literal);
}

bool MIDILang::SymbolTable::reassignVariable(MIDILang::Token identifier, int value) {
    if (this->variables.find(identifier.literal) == this->variables.end()) return false;

    this->variables.erase(identifier.literal);
    this->variables.insert({ identifier.literal, value });
    return true;
}

void MIDILang::SymbolTable::dropVariable(MIDILang::Token identifier) {
    this->variables.erase(identifier.literal);
}

bool MIDILang::SymbolTable::hasVariable(MIDILang::Token identifier) {
    if (this->variables.find(identifier.literal) != this->variables.end()) return true;
    else return false;
}

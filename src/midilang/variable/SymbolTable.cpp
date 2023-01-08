#include "SymbolTable.h"


MIDILang::SymbolTable::SymbolTable() {
    this->scope = 0;
    this->tunes = std::unordered_map<std::string, TuneVar>();
    this->variables = std::unordered_map<std::string, VariableVar>();
}

bool MIDILang::SymbolTable::addTune(MIDILang::Token identifier, MIDILang::Tune tuneStmt) {
    if (this->tunes.find(identifier.literal) != this->tunes.end()) return false;

    this->tunes.insert({ identifier.literal, { tuneStmt, this->scope } });
    return true;
}

MIDILang::Tune MIDILang::SymbolTable::getTune(MIDILang::Token identifier) {
    return this->tunes.at(identifier.literal).getTune();
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

    this->variables.insert({ identifier.literal, { value, this->scope } });
    return true;
}

int MIDILang::SymbolTable::getVariable(MIDILang::Token identifier) {
    return this->variables.at(identifier.literal).getValue();
}

bool MIDILang::SymbolTable::reassignVariable(MIDILang::Token identifier, int value) {
    if (this->variables.find(identifier.literal) == this->variables.end()) return false;

    this->variables.erase(identifier.literal);
    this->variables.insert({ identifier.literal, { value, this->scope } });
    return true;
}

void MIDILang::SymbolTable::dropVariable(MIDILang::Token identifier) {
    this->variables.erase(identifier.literal);
}

bool MIDILang::SymbolTable::hasVariable(MIDILang::Token identifier) {
    if (this->variables.find(identifier.literal) != this->variables.end()) return true;
    else return false;
}

void MIDILang::SymbolTable::dropAndDecrement() {
    // Names Of Variables To Drop
    std::vector<std::string> tuneNames;
    std::vector<std::string> varNames;

    // Searching For All Tunes To Drop
    for (auto& tune: this->tunes) {
        if (tune.second.getScope() == this->scope) tuneNames.push_back(tune.first);
    }

    // Searching For All Variables To Drop
    for (auto& var: this->variables) {
        if (var.second.getScope() == this->scope) varNames.push_back(var.first);
    }

    // Dropping All Marked Tunes
    for (auto& tuneName: tuneNames) {
        this->tunes.erase(tuneName);
    }

    // Dropping All Marked Variables
    for (auto& varName: varNames) {
        this->variables.erase(varName);
    }

    decrementScope(); // Reduce The Scope Depth
}

void MIDILang::SymbolTable::incrementScope() {
    this->scope++;
}

void MIDILang::SymbolTable::decrementScope() {
    this->scope--;
}

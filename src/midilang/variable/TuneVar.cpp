#include "TuneVar.h"


MIDILang::TuneVar::TuneVar(MIDILang::Tune tune, int scope) : tune(tune) {
    this->tune = tune;
    this->scope = scope;
}

MIDILang::Tune MIDILang::TuneVar::getTune() {
    return this->tune;
}

int MIDILang::TuneVar::getScope() {
    return this->scope;
}

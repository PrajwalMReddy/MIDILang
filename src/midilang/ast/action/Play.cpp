#include "Play.h"


MIDILang::Play::Play(MIDILang::Token tune, std::vector<Expression*> arguments) {
    this->tune = tune;
    this->arguments = arguments;
}

std::any MIDILang::Play::accept(MIDILang::ActVisitor &visitor) {
    return visitor.visitPlayAction(this);
}

MIDILang::Token MIDILang::Play::getTune() {
    return this->tune;
}

std::vector<MIDILang::Expression*> MIDILang::Play::getArguments() {
    return this->arguments;
}

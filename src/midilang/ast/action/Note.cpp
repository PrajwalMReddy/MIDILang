#include "Note.h"


MIDILang::Note::Note(MIDILang::Expression* note, MIDILang::Expression* duration, MIDILang::Expression* velocity, int line) {
    this->note = note;
    this->duration = duration;
    this->velocity = velocity;
    this->line = line;
}

std::any MIDILang::Note::accept(MIDILang::ActVisitor& visitor) {
    return visitor.visitNoteAction(this);
}

MIDILang::Expression* MIDILang::Note::getNote() {
    return this->note;
}

MIDILang::Expression* MIDILang::Note::getDuration() {
    return this->duration;
}

MIDILang::Expression* MIDILang::Note::getVelocity() {
    return this->velocity;
}

int MIDILang::Note::getLine() {
    return this->line;
}

#ifndef MIDILANG_NOTE_H
#define MIDILANG_NOTE_H

#include "../Expression.h"
#include "../ActVisitor.h"
#include "../Action.h"


namespace MIDILang {
    class Note: public Action {
        public:
            Note(Expression* note, Expression* duration, Expression* velocity, int line);
            std::any accept(ActVisitor& visitor);

            Expression* getNote();
            Expression* getDuration();
            Expression* getVelocity();

            int getLine();

        private:
            Expression* note;
            Expression* duration;
            Expression* velocity;

            int line;
    };
}


#endif

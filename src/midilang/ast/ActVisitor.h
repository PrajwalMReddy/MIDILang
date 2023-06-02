#ifndef MIDILANG_ACTVISITOR_H
#define MIDILANG_ACTVISITOR_H

#include <any>


namespace MIDILang {
    class ActVisitor {
        public:
            virtual std::any visitAssignmentAction(class Assignment* action) = 0;
            virtual std::any visitIfAction(class If* action) = 0;
            virtual std::any visitLoopAction(class Loop* action) = 0;
            virtual std::any visitPlayAction(class Play* action) = 0;
            virtual std::any visitNoteAction(class Note* action) = 0;
    };
}


#endif

#ifndef MIDILANG_ACTION_H
#define MIDILANG_ACTION_H

#include <any>

#include "ActVisitor.h"


namespace MIDILang {
    class Action {
        public:
            virtual std::any accept(ActVisitor& visitor) = 0;
    };
}


#endif

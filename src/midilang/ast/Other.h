#ifndef MIDILANG_OTHER_H
#define MIDILANG_OTHER_H

#include <any>

#include "OtherVisitor.h"


namespace MIDILang {
    class Other {
        public:
            virtual std::any accept(OtherVisitor& visitor) = 0;
    };
}


#endif

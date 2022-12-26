#ifndef MIDILANG_DECLARATION_H
#define MIDILANG_DECLARATION_H

#include <any>

#include "DeclVisitor.h"


namespace MIDILang {
    class Declaration {
        public:
            virtual std::any accept(DeclVisitor& visitor) = 0;
    };
}


#endif

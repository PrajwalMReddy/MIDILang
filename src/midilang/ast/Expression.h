#ifndef MIDILANG_EXPRESSION_H
#define MIDILANG_EXPRESSION_H

#include <any>

#include "ExprVisitor.h"


namespace MIDILang {
    class Expression {
        public:
            virtual std::any accept(ExprVisitor& visitor) = 0;
    };
}


#endif

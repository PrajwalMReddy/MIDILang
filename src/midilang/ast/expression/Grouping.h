#ifndef MIDILANG_GROUPING_H
#define MIDILANG_GROUPING_H

#include "../Expression.h"
#include "../../lang/Token.h"


namespace MIDILang {
    class Grouping: public Expression {
        public:
            Grouping(Expression* value);
            std::any accept(ExprVisitor& visitor);

            Expression* getValue();

        private:
            Expression* value;
    };
}


#endif

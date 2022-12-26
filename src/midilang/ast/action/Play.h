#ifndef MIDILANG_PLAY_H
#define MIDILANG_PLAY_H

#include <vector>

#include "../Expression.h"
#include "../ActVisitor.h"
#include "../../lang/Token.h"
#include "../Action.h"


namespace MIDILang {
    class Play: public Action {
        public:
            Play(Token tune, std::vector<Expression*> arguments);
            std::any accept(ActVisitor& visitor);

            Token getTune();
            std::vector<Expression*> getArguments();

        private:
            Token tune;
            std::vector<Expression*> arguments;
    };
}


#endif

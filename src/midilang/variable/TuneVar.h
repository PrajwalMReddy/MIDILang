#ifndef MIDILANG_TUNEVAR_H
#define MIDILANG_TUNEVAR_H

#include <string>

#include "../ast/declaration/Tune.h"


namespace MIDILang {
    class TuneVar {
        public:
            TuneVar(MIDILang::Tune tune, int scope);

            Tune getTune();
            int getScope();

        private:
            Tune tune;
            int scope;
    };
}


#endif

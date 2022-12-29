#ifndef MIDILANG_VARIABLEVAR_H
#define MIDILANG_VARIABLEVAR_H

#include <string>


namespace MIDILang {
    class VariableVar {
        public:
            VariableVar(int value, int scope);

            int getValue();
            int getScope();

        private:
            int value;
            int scope;
    };
}


#endif

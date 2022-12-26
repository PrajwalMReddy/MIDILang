#ifndef MIDILANG_ERROR_H
#define MIDILANG_ERROR_H

#include <string>


namespace MIDILang {
    struct Error {
        std::string errorType;
        std::string message;
        int line;
    };
}


#endif

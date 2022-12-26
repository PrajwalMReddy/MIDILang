#ifndef MIDILANG_ERRORHANDLER_H
#define MIDILANG_ERRORHANDLER_H

#include <vector>
#include <iostream>
#include <sstream>

#include "Error.h"


namespace MIDILang {
    class ErrorHandler {
        public:
            ErrorHandler(std::string file);

            void addError(std::string errorType, std::string message, int line);
            void displayIfHasErrors();

        private:
            bool hasErrors();

            static std::vector<std::string> splitString(std::string string);
            static std::string trimString(std::string string);

            std::vector<Error> errors;
            std::string file;
    };
}


#endif

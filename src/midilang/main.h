#ifndef MIDILANG_MAIN_H
#define MIDILANG_MAIN_H

#include <iostream>
#include <fstream>
#include <string>

#include "lang/ErrorHandler.h"
#include "lang/Token.h"
#include "core/Lexer.h"
#include "core/Parser.h"
#include "ast/other/Program.h"
#include "core/Compiler.h"


namespace MIDILang {
    std::string readFile(std::string path);
    void errorMessage(std::string message);
}


#endif

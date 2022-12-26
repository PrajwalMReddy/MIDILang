#include "ErrorHandler.h"


MIDILang::ErrorHandler::ErrorHandler(std::string file) {
    this->errors = std::vector<Error>();
    this->file = file;
}

void MIDILang::ErrorHandler::addError(std::string errorType, std::string message, int line) {
    this->errors.push_back({ errorType, message, line });
}

void MIDILang::ErrorHandler::displayIfHasErrors() {
    if (!hasErrors()) return;

    std::cout << "\n-- Errors --\n" << std::endl;
    std::vector<std::string> lines = splitString(this->file);

    for (Error error: this->errors) {
        std::cout << error.errorType << " | " << error.message << std::endl;
        std::cout << "Line " << error.line << ": " << (((int) lines.size() > 0) ? trimString(lines[error.line - 1]) : "") << "\n" << std::endl;
    }

    exit(1);
}

bool MIDILang::ErrorHandler::hasErrors() {
    return !this->errors.empty();
}

std::vector<std::string> MIDILang::ErrorHandler::splitString(std::string sourceCode) {
    std::vector<std::string> lines;

    const char* charSourceCode = sourceCode.c_str();
    std::stringstream stringStreamSourceCode = std::stringstream(charSourceCode);

    std::string temp;

    while (std::getline(stringStreamSourceCode, temp, '\n')) {
        lines.push_back(temp);
    }

    return lines;
}

std::string MIDILang::ErrorHandler::trimString(std::string string) {
    int startIndex = 0;

    for (int i = 0; i < string.size(); i++) {
        if (string[i] != ' ' && string[i] != '\t') {
            startIndex = i;
            break;
        }
    }

    return string.substr(startIndex, string.size());
}

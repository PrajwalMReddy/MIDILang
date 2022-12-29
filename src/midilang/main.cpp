#include "main.h"


int main(int argc, char* argv[]) {
    if (argc == 1) MIDILang::errorMessage("A File Path Must Be Provided");

    // Set Up
    std::string path = argv[1];
    std::string stem = path.substr(0, path.find_last_of('.'));
    std::string contents = MIDILang::readFile(path);
    MIDILang::ErrorHandler* errors = new MIDILang::ErrorHandler(contents);

    // Compiler Pipeline
    std::vector<MIDILang::Token> tokens = MIDILang::Lexer(contents, errors).lex(); errors->displayIfHasErrors();
    MIDILang::Program* program = MIDILang::Parser(tokens, errors).parse(); errors->displayIfHasErrors();
    MIDILang::Compiler(program, stem, errors).compile(); errors->displayIfHasErrors();

    std::cout << "\nSuccessfully Generated " << stem << ".mid" << std::endl;
    return 0;
}

std::string MIDILang::readFile(std::string path) {
    std::ifstream file;
    file.open(path);

    std::string stringOut;
    std::string temp;

    if (file.is_open()) {
        while (getline(file, temp)) stringOut += temp + "\n";
        return stringOut;
    } else {
        errorMessage("Unable To Read File " + path);
        return "";
    }
}

void MIDILang::errorMessage(std::string message) {
    std::cout << "\n" << message << std::endl;
    exit(1);
}

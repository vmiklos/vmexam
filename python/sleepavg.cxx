#include <fstream>
#include <iostream>
#include <sstream>

/// Quick&dirty C++ port of the Python version that doesn't run on Python 3.4.

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        std::cerr << "usage: sleepavg <input csv>" << std::endl;
        return 1;
    }

    int count = 0;
    double sum = 0;

    std::ifstream csvfile(argv[1]);
    if (!csvfile.is_open())
    {
        std::cerr << "failed to open " << argv[1] << std::endl;
        return 1;
    }

    std::string line;
    bool first = true;
    while (std::getline(csvfile, line))
    {
        std::stringstream ss(line);
        std::string token;
        for (int i = 0; i < 4; ++i)
        {
            std::getline(ss, token, ',');
        }
        if (first)
        {
            first = false;
            if (token != "Duration(sec)")
            {
                std::cerr
                    << "expected 4th col in first row to be 'Duration(sec)'"
                    << std::endl;
                return 1;
            }

            continue;
        }

        ++count;
        sum += std::stoi(token);
    }

    double avg = sum / count;
    int hours = avg / 60 / 60;
    avg -= hours * 60 * 60;
    int minutes = avg / 60;
    avg -= minutes * 60;
    double seconds = avg;
    std::cerr << "Average is " << hours << ":";
    // Using std::setfill() / std::setw() would only work for minutes, not for
    // seconds, which is not int.
    if (minutes < 10)
        std::cerr << "0";
    std::cerr << minutes << ":";
    if (seconds < 10)
        std::cerr << "0";
    std::cerr << seconds << " (" << count << " nigths)" << std::endl;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */

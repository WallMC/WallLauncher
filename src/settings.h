#include <iostream>
#include <cstdlib>
#include <boost/filesystem.hpp>

using namespace std;
using namespace boost::filesystem;

void check_config() {
  string conf_dir;

  conf_dir = getenv("HOME");

  conf_dir.append("/.config/wallmc/launcher.conf");

 if ( ! exists(conf_dir) ) {
     cout << "using default settings file";
 }
 

  
}

#include <iostream>
#include <cstring>
#include "help.h"
#include "settings.h"
using namespace std;

int main(int argc, char *argv[]) {
    switch ( argc ) {
     case 1:
	     // run program as normal
	     check_config();
	     break;
     case 2:
	    if ( strncmp( argv[1], "-h" , 2 ) == 0 ) 
		help();
	    
	    else if ( strncmp( argv[1], "--help" , 6 ) == 0 )
		help();
	    else
		 cout << "error: unknown option\n";
	        // help();
	    break;
    }

  if ( argc > 2 ) {
	cout << "error: only one option is supported\n";
  }

  return 0;
}

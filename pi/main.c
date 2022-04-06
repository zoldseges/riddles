#include <gmp.h>

int main(int argc, char *argv[]) {
  mpz_t i;
  mpz_init(i);
  mpz_set_str(i, argv[1], 10);
  mpz_fac_ui(i, mpz_get_ui(i));
  gmp_printf("%Zd\n",i);
}

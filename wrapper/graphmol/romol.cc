#include "rdkit-rust-ffi/wrapper/graphmol/romol.h"

namespace RDKit {
std::unique_ptr<ROMol> newMolFromSmiles(const std::string &smiles) {
	ROMol *mol = SmilesToMol(smiles);
	return std::unique_ptr<ROMol>(mol);
}
unsigned int getAtomDegree(const ROMol &mol, const Atom &atom) { return mol.getAtomDegree(&atom); }
} // namespace RDKit
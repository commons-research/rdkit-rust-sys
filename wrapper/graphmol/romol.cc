#include "rdkit-rust-ffi/wrapper/graphmol/romol.h"

namespace RDKit {
std::unique_ptr<ROMol> newMolFromSmiles(const std::string &smiles) {
	ROMol *mol = SmilesToMol(smiles);
	return std::unique_ptr<ROMol>(mol);
}
std::shared_ptr<ROMol> newMolFromSmilesAsSharedPtr(const std::string &smiles) {
	ROMol *mol = SmilesToMol(smiles);
	return std::shared_ptr<ROMol>(mol);
}
unsigned int getAtomDegree(const ROMol &mol, const Atom &atom) { return mol.getAtomDegree(&atom); }
const Atom &getAtomWithIdx(const RDKit::ROMol &mol, unsigned int idx) { return *mol.getAtomWithIdx(idx); }
Atom &getAtomWithIdx(RDKit::ROMol &mol, unsigned int idx) { return *mol.getAtomWithIdx(idx); }
Atom &getAtomWithBookmark(RDKit::ROMol &mol, int mark) { return *mol.getAtomWithBookmark(mark); }
} // namespace RDKit
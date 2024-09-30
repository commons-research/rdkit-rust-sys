#include "rdkit-rust-ffi/wrapper/graphmol/atom.h"

namespace RDKit {
std::shared_ptr<Atom> make_shared(std::unique_ptr<Atom> atom) { return std::shared_ptr<Atom>(atom.release()); }
std::unique_ptr<Atom> newAtom() { return std::unique_ptr<Atom>(new Atom()); }
std::unique_ptr<Atom> newAtomFromAtomicNum(int atomicNum) { return std::unique_ptr<Atom>(new Atom(atomicNum)); }
std::unique_ptr<Atom> newAtomFromSymbol(const std::string &symbol) { return std::unique_ptr<Atom>(new Atom(symbol)); }
std::unique_ptr<Atom> newAtomFromOther(const Atom &other) { return std::unique_ptr<Atom>(new Atom(other)); }
// void setAtomAtomicNum(Atom &atom, int atomicNum) { atom.setAtomicNum(atomicNum); }
// Define the getSymbol method for Atom
rust::String getSymbolAsString(const Atom &atom) { return atom.getSymbol(); }
bool MatchRust(const Atom &atom, std::unique_ptr<Atom> other) { return atom.Match(other.get()); }
int calcExplicitValence(Atom &atom) { return atom.calcExplicitValence(); }
int calcImplicitValence(Atom &atom) { return atom.calcImplicitValence(); }

using ChiralType = Atom::ChiralType;
using HybridizationType = Atom::HybridizationType;

rust::String getQueryTypeRust(const Atom &atom) { return atom.getQueryType(); }
} // namespace RDKit

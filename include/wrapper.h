#pragma once
#include "rust/cxx.h"
#include <GraphMol/Atom.h>
#include <iostream>
#include <memory>

namespace RDKit {
std::shared_ptr<Atom> make_shared(std::unique_ptr<Atom> atom);
std::unique_ptr<Atom> newAtom();
std::unique_ptr<Atom> newAtomFromAtomicNum(int atomicNum);
std::unique_ptr<Atom> newAtomFromSymbol(const std::string &symbol);
std::unique_ptr<Atom> newAtomFromOther(const Atom &other);
// void setAtomAtomicNum(Atom &atom, int atomicNum);

// Extend Atom to expose getSymbol to Rust
rust::String getSymbolAsString(const Atom &atom);
bool MatchRust(const Atom &atom, std::unique_ptr<Atom> other);
int calcExplicitValence(Atom &atom, bool strict = true);
int calcImplicitValence(Atom &atom, bool strict = true);


using ChiralType = Atom::ChiralType;
using HybridizationType = Atom::HybridizationType;

rust::String getQueryTypeRust(const Atom &atom);
} // namespace RDKit

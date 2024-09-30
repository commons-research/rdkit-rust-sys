#pragma once
#include "rust/cxx.h"
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/FileParsers/FileWriters.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>
#include <GraphMol/MolStandardize/Tautomer.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <iostream>


namespace RDKit {
    std::unique_ptr<ROMol> newMolFromSmiles(const std::string &smiles);
    unsigned int getAtomDegree(const ROMol &mol, const Atom &atom);
}
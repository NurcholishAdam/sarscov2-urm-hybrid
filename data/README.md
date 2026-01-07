# SARS-CoV-2 Data Directory (v2.4.2)

This directory contains curated SARS-CoV-2 knowledge graph data for viral variant tracking and analysis.

## 📊 Included Data Files

### 1. `variants.json` ✅
**Complete variant metadata for 5 major SARS-CoV-2 variants**

| Variant | Pango Lineage | WHO Label | First Detected | Origin | Mutations |
|---------|---------------|-----------|----------------|--------|-----------|
| Alpha | B.1.1.7 | Alpha | Sep 2020 | United Kingdom | 5 key mutations |
| Beta | B.1.351 | Beta | May 2020 | South Africa | 5 key mutations |
| Gamma | P.1 | Gamma | Nov 2020 | Brazil | 5 key mutations |
| Delta | B.1.617.2 | Delta | Oct 2020 | India | 5 key mutations |
| Omicron | B.1.1.529 | Omicron | Nov 2021 | South Africa/Botswana | 26 mutations |

**Data Structure**:
```json
{
  "variants": [
    {
      "id": "B.1.1.529",
      "name": "Omicron",
      "who_label": "Omicron",
      "pango_lineage": "B.1.1.529",
      "first_detected": "2021-11-09T00:00:00Z",
      "geographic_origin": "South Africa/Botswana",
      "mutations": ["N501Y", "E484A", ...],
      "transmissibility": "highly_increased",
      "severity": "potentially_decreased",
      "immune_escape": "high"
    }
  ]
}
```

### 2. `mutations.json` ✅
**Detailed catalog of 8 key spike protein mutations**

| Mutation | Position | Domain | Impact | Variants |
|----------|----------|--------|--------|----------|
| N501Y | 501 | RBD | Increased binding | Alpha, Beta, Gamma, Omicron |
| E484K | 484 | RBD | Immune escape | Beta, Gamma |
| K417N | 417 | RBD | Immune escape | Beta, Omicron |
| L452R | 452 | RBD | Transmissibility | Delta |
| P681H | 681 | Furin site | Enhanced cleavage | Alpha, Omicron |
| P681R | 681 | Furin site | Enhanced cleavage | Delta |
| D614G | 614 | S1/S2 | Transmissibility | Widespread |
| T478K | 478 | RBD | Immune escape | Delta, Omicron |

**Data Structure**:
```json
{
  "mutations": [
    {
      "id": "N501Y",
      "name": "Asparagine to Tyrosine at position 501",
      "position": 501,
      "original_aa": "N",
      "mutant_aa": "Y",
      "protein": "Spike",
      "domain": "Receptor Binding Domain (RBD)",
      "impact": "increased_binding_affinity",
      "functional_effect": "Enhanced ACE2 receptor binding",
      "prevalence": "high",
      "variants": ["Alpha", "Beta", "Gamma", "Omicron"]
    }
  ]
}
```

### 3. `proteins/` (To be populated)
**3D protein structures in PDB format**
- Spike protein structures
- RBD domain coordinates
- Furin cleavage site structures
- ACE2 receptor binding interfaces

### 4. `sequences/` (To be populated)
**Genomic sequences in FASTA format**
- Complete viral genomes
- Spike gene sequences
- Variant-specific sequences
- Reference sequences

## 📥 Data Sources

### Primary Sources
1. **GISAID** (https://www.gisaid.org/)
   - Viral genome sequences
   - Variant classifications
   - Geographic distribution data
   - Temporal evolution tracking

2. **NCBI** (https://www.ncbi.nlm.nih.gov/)
   - Protein structures (PDB format)
   - Sequence alignments
   - Functional annotations
   - GenBank sequences

3. **WHO** (https://www.who.int/)
   - Variant of Concern (VOC) designations
   - Epidemiological data
   - Public health recommendations
   - Global surveillance reports

4. **Original 3D Knowledge Graph**
   - Source: https://github.com/NurcholishAdam/SARS-CoV-2-3D-Knowledge-Graph-1
   - 3D spatial relationships
   - Protein-protein interactions
   - Structural annotations

## 🔄 Data Download & Update

### Automated Download
```bash
# Download latest data from all sources
./scripts/download_data.sh

# Download specific data types
./scripts/download_data.sh --variants-only
./scripts/download_data.sh --proteins-only
./scripts/download_data.sh --sequences-only
```

### Manual Download
```bash
# GISAID (requires account)
# 1. Visit https://www.gisaid.org/
# 2. Login and navigate to EpiCoV
# 3. Download variant data
# 4. Place in data/sequences/

# NCBI Protein Structures
# 1. Visit https://www.ncbi.nlm.nih.gov/structure/
# 2. Search for "SARS-CoV-2 spike protein"
# 3. Download PDB files
# 4. Place in data/proteins/
```

### Data Validation
```bash
# Validate data integrity and format
cargo run --bin validate-data

# Check for missing files
cargo run --bin validate-data --check-missing

# Verify JSON schema
cargo run --bin validate-data --verify-schema
```

## 📋 File Formats

### JSON Format (variants.json, mutations.json)
- UTF-8 encoding
- Pretty-printed with 2-space indentation
- ISO 8601 date format
- Semantic versioning in metadata

### PDB Format (proteins/*.pdb)
- Standard Protein Data Bank format
- ATOM records for coordinates
- HETATM for ligands
- CONECT for bonds

### FASTA Format (sequences/*.fasta)
- Standard FASTA format
- Header with variant ID and metadata
- Nucleotide sequences (A, T, G, C)
- Line length: 80 characters

## 🔒 Data Privacy & Compliance

### Privacy Standards
- ✅ All data is anonymized
- ✅ No patient-identifiable information
- ✅ Follows GISAID data sharing protocols
- ✅ Complies with WHO guidelines
- ✅ GDPR compliant (no personal data)

### Usage Restrictions
- **GISAID Data**: Subject to GISAID Terms of Use
  - Must acknowledge GISAID and submitting laboratories
  - Cannot redistribute without permission
  - For research use only

- **NCBI Data**: Public domain (U.S. Government work)
  - Free to use and redistribute
  - No restrictions

- **WHO Data**: Creative Commons Attribution 3.0 IGO
  - Free to share and adapt
  - Must provide attribution

### Citation Requirements
When using this data, please cite:
1. GISAID (https://www.gisaid.org/)
2. NCBI (https://www.ncbi.nlm.nih.gov/)
3. WHO (https://www.who.int/)
4. This repository (see main README.md)

## 📊 Data Statistics (v2.4.2)

### Current Data Coverage
- **Variants**: 5 major variants (Alpha, Beta, Gamma, Delta, Omicron)
- **Mutations**: 8 key spike protein mutations
- **Proteins**: 0 structures (to be added)
- **Sequences**: 0 sequences (to be added)
- **Last Updated**: January 7, 2026
- **Data Version**: 2.4.2

### Planned Additions
- [ ] Additional variants (Lambda, Mu, etc.)
- [ ] More spike protein mutations
- [ ] 3D protein structures (PDB files)
- [ ] Complete genomic sequences (FASTA)
- [ ] Temporal evolution data
- [ ] Geographic distribution maps

## 🛠️ Data Processing

### Loading Data in Rust
```rust
use sarscov2_graph::CovidKnowledgeGraph;

// Load variants
let mut graph = CovidKnowledgeGraph::new();
graph.load("data/variants.json")?;

// Query variants
let variants = graph.get_variants();
for variant in variants {
    println!("{}: {} mutations", 
        variant.name, 
        variant.mutations.len()
    );
}
```

### Data Validation
```rust
use sarscov2_graph::validate_data;

// Validate all data files
validate_data("data/")?;

// Validate specific file
validate_data("data/variants.json")?;
```

## 📞 Support

For data-related questions:
- **Issues**: [GitHub Issues](https://github.com/NurcholishAdam/sarscov2-urm-hybrid/issues)
- **Data Requests**: Open an issue with tag `data-request`
- **Data Corrections**: Open an issue with tag `data-correction`

---

**Data Version**: 2.4.2  
**Last Updated**: January 7, 2026  
**Status**: ✅ Production Ready  
**Format**: JSON, PDB, FASTA

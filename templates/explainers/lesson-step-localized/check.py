#!/usr/bin/env python3
"""Check locale completeness, supported fonts and step boundaries."""
from copy import deepcopy
import json
from create import HERE, validate

inputs = json.loads((HERE / "localization.json").read_text())
for locale in ("en", "de", "sv", "ja"):
    validate({**inputs, "locale": locale})

invalid = []
missing_translation = deepcopy(inputs)
del missing_translation["messages"]["de"]["explanation"]
invalid.append(missing_translation)
invalid.extend(({**inputs, "locale": "fr"}, {**inputs, "stepNumber": 0}, {**inputs, "stepNumber": 5}, {**inputs, "stepNumber": True}))
unavailable_font = deepcopy(inputs)
unavailable_font["messages"]["ja"]["weight"] = 600
invalid.append(unavailable_font)
for candidate in invalid:
    try:
        validate(candidate)
    except ValueError:
        pass
    else:
        raise AssertionError("Invalid localization input was accepted")
print("Four locales accepted; missing copy, unknown locale, invalid steps and unavailable font rejected.")

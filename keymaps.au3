Local $actionMap = ObjCreate("Scripting.Dictionary")
If @error Then
    MsgBox(0, '', 'Error creating the dictionary object')
EndIf

; Add keybinds below
$actionMap.Add("Comfort Zone",              "^2")
$actionMap.Add("Inner Quiet",               "^3")
$actionMap.Add("Steady Hand II",            "^4")
$actionMap.Add("Precise Touch",             "^5")
$actionMap.Add("Basic Touch",               "^6")
$actionMap.Add("Both Touch",                "^5^6")
$actionMap.Add("Manipulation",              "^7")
$actionMap.Add("Piece by Piece",            "#")
$actionMap.Add("Steady Hand",               "^w")
$actionMap.Add("Innovation",                "^e")
$actionMap.Add("Great Strides",             "^r")
$actionMap.Add("Ingenuity II",              "^t")
$actionMap.Add("Byregots Blessing",         "^v")
$actionMap.Add("Byregot's Blessing",        "^v")
$actionMap.Add("Careful Synthesis II",      "^b")
$actionMap.Add("Careful Synthesis III",     "W")
$actionMap.Add("Standard Touch",            "^s")
$actionMap.Add("Advanced Touch",            "^d")
$actionMap.Add("Waste Not II",              "^f")
$actionMap.Add("Master's Mend",             "^g")
$actionMap.Add("Tricks of the Trade",       "^z")
$actionMap.Add("Standard Synthesis",        "^x")
$actionMap.Add("Basic Synthesis",           "^c")
$actionMap.Add("Master's Mend II",          "X")
$actionMap.Add("Hasty Touch",               "C")
pwd() # Confirm current directory Current Directory
required_skills = ["Supply Chain", "Project"] #  to be scanned in the given CV
src = "HasanResume.pdf" # CV to be analized
using PDFIO
function getPDFText(src, out)
    doc = pdDocOpen(src)
    docinfo = pdDocGetInfo(doc)
    open(out, "w") do io
		npage = pdDocGetPageCount(doc)
            for i=1:npage
                page = pdDocGetPage(doc, i)
                pdPageExtractText(io, page)
            end
    end
    pdDocClose(doc)
    return docinfo
end
getPDFText(src, "$src.txt")
rx = "skill"
for skill in required_skills
    rx = "$rx|$skill"
end
rx = Regex(string("($rx)s?"))
cv_text = read("$src.txt", String)
found = collect(eachmatch(rx, cv_text, overlap = true))
found_skills = []
for (i, entry) in enumerate(found)
    push!(found_skills, entry.match)
end
sort!(found_skills)
for i in reverse(1:length(found_skills))
    if i>1 && found_skills[i] == found_skills[i-1] 
        deleteat!(found_skills, i) 
    end
end
found_skills
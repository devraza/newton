package main

import (
	"encoding/json"
	"fmt"
	"io"
	"io/fs"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

type Response struct {
	Status string `json:"status"`
	Result Result `json:"result"`
}

type Result struct {
	Problems          []Problem           `json:"problems"`
	ProblemStatistics []ProblemStatistics `json:"problemStatistics"`
}

type Problem struct {
	ContestId   int     `json:"contestId"`
	Name        string  `json:"name"`
	Index       string  `json:"index"`
	Type        string  `json:"type"`
	Points      float32 `json:"points"`
	Rating      int     `json:"rating"`
	SolvedCount int     `json:"solvedCount"`
}

type ProblemStatistics struct {
	ContestId   int    `json:"contestId"`
	Index       string `json:"index"`
	SolvedCount int    `json:"solvedCount"`
}

var files []string

func visit(path string, d fs.DirEntry, err error) error {
	if err != nil {
		return err
	}
	if d.IsDir() && d.Name() == "target" {
		return filepath.SkipDir
	}
	files = append(files, strings.TrimSuffix(d.Name(), filepath.Ext(d.Name())))
	return nil
}

func main() {
	resp, err := http.Get("https://codeforces.com/api/problemset.problems")

	if err != nil {
		log.Fatalln(err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalln(err)
	}

	var response Response
	if err := json.Unmarshal(body, &response); err != nil {
		log.Fatalln(err)
	}

	err = filepath.WalkDir("../solutions", visit)

	var solved []string

	for _, p := range response.Result.Problems {
		for _, i := range files {
      combined := fmt.Sprintf("%v%v", p.ContestId, p.Index)
			if i == combined {
				text := fmt.Sprintf("- [%v %v](https://codeforces.com/problemset/problem/%v/%v)", combined, p.Name, p.ContestId, p.Index)
				solved = append(solved, text)
				continue
			}
		}
	}

	file, err := os.Create("../README.md")
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer file.Close()

	file.WriteString("# Newton\nMy set of solutions to the problems on Code Forces.\n## Solutions\n")

	for _, line := range solved {
		_, err := file.WriteString(line + "\n")
		if err != nil {
			fmt.Println("Error writing to file:", err)
			return
		}
	}
}

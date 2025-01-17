package main

import (
        "encoding/json"
        "fmt"
        "io"
        "log"
        "net/http"
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

        for _, p := range response.Result.Problems {
                fmt.Printf("%v", p.Name)
        }
}

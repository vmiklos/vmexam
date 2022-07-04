package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/mattn/go-sqlite3"
	"github.com/spf13/cobra"
)

func newCreateCommand(db *sql.DB) *cobra.Command {
	var machine string
	var service string
	var user string
	var password string
	var cmd = &cobra.Command{
		Use:   "create",
		Short: "creates a new password",
		RunE: func(cmd *cobra.Command, args []string) error {
			query, err := db.Prepare("insert into passwords (machine, service, user, password) values(?, ?, ?, ?)")
			if err != nil {
				return fmt.Errorf("db.Prepare(insert) failed: %s", err)
			}

			_, err = query.Exec(machine, service, user, password)

			return nil
		},
	}
	cmd.Flags().StringVarP(&machine, "machine", "m", "", "machine (required)")
	cmd.MarkFlagRequired("machine")
	cmd.Flags().StringVarP(&service, "service", "s", "", "service (required)")
	cmd.MarkFlagRequired("service")
	cmd.Flags().StringVarP(&user, "user", "u", "", "user (required)")
	cmd.MarkFlagRequired("user")
	cmd.Flags().StringVarP(&password, "password", "p", "", "password (required)")
	cmd.MarkFlagRequired("password")

	return cmd
}

func newRootCommand(db *sql.DB) *cobra.Command {
	var cmd = &cobra.Command{
		Use:   "cpm",
		Short: "cpm is a console password manager",
		RunE: func(cmd *cobra.Command, args []string) error {
			fmt.Println("rootCmd.Run")
			return nil
		},
	}
	cmd.AddCommand(newCreateCommand(db))

	return cmd
}

func newDatabase() (*sql.DB, error) {
	db, err := sql.Open("sqlite3", "./cpmdb")
	if err != nil {
		log.Fatalf("sql.Open() failed: %s", err)
	}

	query, err := db.Prepare(`create table if not exists passwords (
		id integer primary key,
		machine text not null,
		service text not null,
		user text not null,
		password text not null
	)`)
	if err != nil {
		return nil, err
	}
	query.Exec()

	return db, nil
}

func main() {
	db, err := newDatabase()
	if err != nil {
		log.Fatalf("newDatabase() failed: %s", err)
	}
	defer db.Close()

	var cmd = newRootCommand(db)
	err = cmd.Execute()
	if err != nil {
		log.Fatalf("rootCmd.Execute() failed: %s", err)
	}
}

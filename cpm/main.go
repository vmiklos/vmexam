package main

import (
	"database/sql"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/exec"

	_ "github.com/mattn/go-sqlite3"
	"github.com/spf13/cobra"
)

func newCreateCommand(db *sql.DB) *cobra.Command {
	var machine string
	var service string
	var user string
	var password string
	var passwordType string
	var cmd = &cobra.Command{
		Use:   "create",
		Short: "creates a new password",
		RunE: func(cmd *cobra.Command, args []string) error {
			query, err := db.Prepare("insert into passwords (machine, service, user, password, type) values(?, ?, ?, ?, ?)")
			if err != nil {
				return fmt.Errorf("db.Prepare() failed: %s", err)
			}

			_, err = query.Exec(machine, service, user, password, passwordType)
			if err != nil {
				return fmt.Errorf("query.Exec() failed: %s", err)
			}

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
	cmd.Flags().StringVarP(&passwordType, "type", "t", "plain", "password type ('plain' or 'totp', default: plain)")

	return cmd
}

func newUpdateCommand(db *sql.DB) *cobra.Command {
	var machine string
	var service string
	var user string
	var password string
	var passwordType string
	var cmd = &cobra.Command{
		Use:   "update",
		Short: "updates an existing password",
		RunE: func(cmd *cobra.Command, args []string) error {
			query, err := db.Prepare("update passwords set password=? where machine=? and service=? and user=? and type=?")
			if err != nil {
				return fmt.Errorf("db.Prepare(update) failed: %s", err)
			}

			_, err = query.Exec(password, machine, service, user, passwordType)

			return nil
		},
	}
	cmd.Flags().StringVarP(&machine, "machine", "m", "", "machine (required)")
	cmd.MarkFlagRequired("machine")
	cmd.Flags().StringVarP(&service, "service", "s", "", "service (required)")
	cmd.MarkFlagRequired("service")
	cmd.Flags().StringVarP(&user, "user", "u", "", "user (required)")
	cmd.MarkFlagRequired("user")
	cmd.Flags().StringVarP(&password, "password", "p", "", "new password (required)")
	cmd.MarkFlagRequired("password")
	cmd.Flags().StringVarP(&passwordType, "type", "t", "plain", "password type ('plain' or 'totp', default: plain)")

	return cmd
}

func newDeleteCommand(db *sql.DB) *cobra.Command {
	var machine string
	var service string
	var user string
	var passwordType string
	var cmd = &cobra.Command{
		Use:   "delete",
		Short: "deletes an existing password",
		RunE: func(cmd *cobra.Command, args []string) error {
			query, err := db.Prepare("delete from passwords where machine=? and service=? and user=? and type=?")
			if err != nil {
				return fmt.Errorf("db.Prepare(delete) failed: %s", err)
			}

			_, err = query.Exec(machine, service, user, passwordType)

			return nil
		},
	}
	cmd.Flags().StringVarP(&machine, "machine", "m", "", "machine (required)")
	cmd.MarkFlagRequired("machine")
	cmd.Flags().StringVarP(&service, "service", "s", "", "service (required)")
	cmd.MarkFlagRequired("service")
	cmd.Flags().StringVarP(&user, "user", "u", "", "user (required)")
	cmd.MarkFlagRequired("user")
	cmd.Flags().StringVarP(&passwordType, "type", "t", "plain", "password type ('plain' or 'totp', default: plain)")

	return cmd
}

func newReadCommand(db *sql.DB) *cobra.Command {
	var machineFlag string
	var serviceFlag string
	var userFlag string
	var typeFlag string
	var cmd = &cobra.Command{
		Use:   "search",
		Short: "searches passwords",
		RunE: func(cmd *cobra.Command, args []string) error {
			rows, err := db.Query("select machine, service, user, password, type from passwords")
			if err != nil {
				return fmt.Errorf("db.Query(insert) failed: %s", err)
			}

			defer rows.Close()
			for rows.Next() {
				var machine string
				var service string
				var user string
				var password string
				var passwordType string
				err = rows.Scan(&machine, &service, &user, &password, &passwordType)
				if err != nil {
					return fmt.Errorf("rows.Scan() failed: %s", err)
				}

				if len(machineFlag) > 0 && machine != machineFlag {
					continue
				}

				if len(serviceFlag) > 0 && service != serviceFlag {
					continue
				}

				if len(userFlag) > 0 && user != userFlag {
					continue
				}

				fmt.Printf("%s %s@%s %s\n", service, user, machine, password)
			}

			return nil
		},
	}
	cmd.Flags().StringVarP(&machineFlag, "machine", "m", "", "machine (required)")
	cmd.Flags().StringVarP(&serviceFlag, "service", "s", "", "service (required)")
	cmd.Flags().StringVarP(&userFlag, "user", "u", "", "user (required)")
	cmd.Flags().StringVarP(&typeFlag, "type", "t", "plain", "password type ('plain' or 'totp', default: plain)")

	return cmd
}

func newRootCommand(db *sql.DB) *cobra.Command {
	var cmd = &cobra.Command{
		Use:   "cpm",
		Short: "cpm is a console password manager",
	}
	cmd.AddCommand(newCreateCommand(db))
	cmd.AddCommand(newReadCommand(db))
	cmd.AddCommand(newUpdateCommand(db))
	cmd.AddCommand(newDeleteCommand(db))

	return cmd
}

func getCommands() []string {
	return []string{"create", "read", "update", "delete"}
}

// CpmDatabase is an opened tempfile, containing an sqlite database.
type CpmDatabase struct {
	File     *os.File
	Database *sql.DB
}

func pathExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func openDatabase() (*CpmDatabase, error) {
	var db CpmDatabase
	var err error
	db.File, err = ioutil.TempFile("", "cpm")
	if err != nil {
		return nil, fmt.Errorf("ioutil.TempFile() failed: %s", err)
	}

	if pathExists("./cpmdb") {
		os.Remove(db.File.Name())
		cmd := exec.Command("gpg", "--decrypt", "-a", "-o", db.File.Name(), "./cpmdb")
		err := cmd.Start()
		if err != nil {
			return nil, fmt.Errorf("cmd.Start() failed: %s", err)
		}
		err = cmd.Wait()
		if err != nil {
			return nil, fmt.Errorf("cmd.Wait() failed: %s", err)
		}
	}

	db.Database, err = sql.Open("sqlite3", db.File.Name())
	if err != nil {
		return nil, fmt.Errorf("sql.Open() failed: %s", err)
	}

	query, err := db.Database.Prepare(`create table if not exists passwords (
		id integer primary key,
		machine text not null,
		service text not null,
		user text not null,
		password text not null,
		type text not null
	)`)
	if err != nil {
		return nil, err
	}
	query.Exec()

	return &db, nil
}

func closeDatabase(db *CpmDatabase) {
	db.Database.Close()

	os.Remove("./cpmdb")
	// TODO harcoded uid
	cmd := exec.Command("gpg", "--encrypt", "--sign", "-a", "-r", "03915096", "-o", "./cpmdb", db.File.Name())
	err := cmd.Start()
	if err != nil {
		log.Fatalf("cmd.Start(gpg encrypt) failed: %s", err)
	}
	err = cmd.Wait()
	if err != nil {
		log.Fatalf("cmd.Wait(gpg encrypt) failed: %s", err)
	}

	os.Remove(db.File.Name())
}

func main() {
	db, err := openDatabase()
	if err != nil {
		log.Fatalf("openDatabase() failed: %s", err)
	}
	defer closeDatabase(db)

	var commandFound bool
	commands := getCommands()
	for _, a := range commands {
		for _, b := range os.Args[1:] {
			if a == b {
				commandFound = true
				break
			}
		}
	}
	var cmd = newRootCommand(db.Database)
	if !commandFound {
		// Default to the search subcommand.
		args := append([]string{"search"}, os.Args[1:]...)
		cmd.SetArgs(args)
	}

	err = cmd.Execute()
	if err != nil {
		log.Fatalf("rootCmd.Execute() failed: %s", err)
	}
}

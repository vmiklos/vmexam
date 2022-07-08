package main

import (
	"bytes"
	"database/sql"
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"os/user"
	"strings"

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

// XMLPassword is the 4th <node> element from cpm's XML database.
type XMLPassword struct {
	XMLName xml.Name `xml:"node"`
	Label   string   `xml:"label,attr"`
	Totp    string   `xml:"totp,attr"`
}

// XMLUser is the 3rd <node> element from cpm's XML database.
type XMLUser struct {
	XMLName   xml.Name      `xml:"node"`
	Label     string        `xml:"label,attr"`
	Passwords []XMLPassword `xml:"node"`
}

// XMLService is the 2nd <node> element from cpm's XML database.
type XMLService struct {
	XMLName xml.Name  `xml:"node"`
	Label   string    `xml:"label,attr"`
	Users   []XMLUser `xml:"node"`
}

// XMLMachine is the 1st <node> element from cpm's XML database.
type XMLMachine struct {
	XMLName  xml.Name     `xml:"node"`
	Label    string       `xml:"label,attr"`
	Services []XMLService `xml:"node"`
}

// XMLMachines is the <root> element from cpm's XML database.
type XMLMachines struct {
	XMLName  xml.Name     `xml:"root"`
	Machines []XMLMachine `xml:"node"`
}

func newImportCommand(db *sql.DB) *cobra.Command {
	var cmd = &cobra.Command{
		Use:   "import",
		Short: "imports an old XML database",
		RunE: func(cmd *cobra.Command, args []string) error {
			// Decrypt and uncompress ~/.cpmdb to a temp file.
			usr, err := user.Current()
			if err != nil {
				return fmt.Errorf("user.Current() failed: %s", err)
			}

			encryptedPath := usr.HomeDir + "/.cpmdb"
			decryptedFile, err := ioutil.TempFile("", "cpm")
			decryptedPath := decryptedFile.Name()
			defer os.Remove(decryptedPath)
			if err != nil {
				return fmt.Errorf("ioutil.TempFile() failed: %s", err)
			}

			os.Remove(decryptedPath)
			gpg := exec.Command("gpg", "--decrypt", "-a", "-o", decryptedPath+".gz", encryptedPath)
			err = gpg.Start()
			if err != nil {
				return fmt.Errorf("cmd.Start() failed: %s", err)
			}
			err = gpg.Wait()
			if err != nil {
				return fmt.Errorf("cmd.Wait() failed: %s", err)
			}

			gunzip := exec.Command("gunzip", decryptedPath+".gz")
			err = gunzip.Start()
			if err != nil {
				return fmt.Errorf("cmd.Start(gunzip) failed: %s", err)
			}
			err = gunzip.Wait()
			if err != nil {
				return fmt.Errorf("cmd.Wait(gunzip) failed: %s", err)
			}

			// Parse the XML.
			xmlFile, err := os.Open(decryptedPath)
			if err != nil {
				return fmt.Errorf("os.Open(decryptedPath) failed: %s", err)
			}
			defer xmlFile.Close()

			xmlBytes, err := ioutil.ReadAll(xmlFile)
			if err != nil {
				return fmt.Errorf("ioutil.ReadAll(xmlFile) failed: %s", err)
			}

			// Avoid 'encoding "ISO-8859-1" declared but Decoder.CharsetReader is nil'.
			xmlBytes = bytes.ReplaceAll(xmlBytes, []byte(`encoding="ISO-8859-1"`), []byte(`encoding="UTF-8"`))

			var machines XMLMachines
			err = xml.Unmarshal(xmlBytes, &machines)
			if err != nil {
				return fmt.Errorf("xml.Unmarshal() failed: %s", err)
			}

			// TODO import the parsed data
			for _, machine := range machines.Machines {
				machineLabel := machine.Label
				for _, service := range machine.Services {
					serviceLabel := service.Label
					for _, user := range service.Users {
						userLabel := user.Label
						for _, password := range user.Passwords {
							passwordLabel := password.Label
							passwordTotp := password.Totp
							fmt.Printf("machine: %s service: %s user: %s password: %s, password type: '%s'\n", machineLabel, serviceLabel, userLabel, passwordLabel, passwordTotp)
						}
					}
				}
			}

			return nil
		},
	}

	return cmd
}

func newReadCommand(db *sql.DB) *cobra.Command {
	var machineFlag string
	var serviceFlag string
	var userFlag string
	var typeFlag string
	var totpFlag bool
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

				if len(typeFlag) > 0 && passwordType != typeFlag {
					continue
				}

				if passwordType == "totp" {
					if totpFlag {
						// This is a TOTP password and the current value is required: invoke
						// oathtool to generate it.
						passwordType = "current totp"
						output, err := exec.Command("oathtool", "-b", "--totp", password).Output()
						if err != nil {
							return fmt.Errorf("exec.Command(oathtool) failed: %s", err)
						}
						password = strings.TrimSpace(string(output))
					} else {
						passwordType = "totp key"
					}
				}

				fmt.Printf("machine: %s service: %s user: %s password: %s, password type: %s\n", machine, service, user, password, passwordType)
			}

			return nil
		},
	}
	cmd.Flags().StringVarP(&machineFlag, "machine", "m", "", "machine (required)")
	cmd.Flags().StringVarP(&serviceFlag, "service", "s", "", "service (required)")
	cmd.Flags().StringVarP(&userFlag, "user", "u", "", "user (required)")
	cmd.Flags().StringVarP(&typeFlag, "type", "t", "plain", "password type ('plain' or 'totp', default: plain)")
	cmd.Flags().BoolVarP(&totpFlag, "totp", "T", false, "show current TOTP, not the TOTP key (default: false)")

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
	cmd.AddCommand(newImportCommand(db))

	return cmd
}

func getCommands() []string {
	return []string{
		"--help",
		"create",
		"delete",
		"import",
		"search",
		"update",
	}
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
		machine text not null,
		service text not null,
		user text not null,
		password text not null,
		type text not null,
		unique(machine, service, user, type)
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

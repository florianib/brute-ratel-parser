pub const MITRE_JSON: &str = r#"{
  "children": [
    {
      "children": [
        {
          "children": [
            {
              "name": "Valid Accounts (T1078 - 002, 003)"
            },
            {
              "name": "Access Token Manipulation (T1134 - 001, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "grab_token"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "lookup"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Application Layer Protocol (T1071 - 001)"
            },
            {
              "name": "Encrypted Channel (T1573)"
            }
          ],
          "name": "Command and Control (TA0011)"
        },
        {
          "children": [
            {
              "name": "Lateral Tool Transfer (T1570)"
            },
            {
              "name": "Remote Services (T1021 - 002)"
            }
          ],
          "name": "Lateral Movement (TA0008)"
        }
      ],
      "name": "pivot_smb"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Create or Modify System Process: Windows Service (T1543 - 003)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "scdelete"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001, 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "coffexec"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Application Window Discovery (T1010"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "windowlist"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Hijack Execution Flow (T1574 - 007)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        },
        {
          "children": [
            {
              "name": "Hijack Execution Flow (T1574 - 007)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "setenv"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "acl"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Impair Defenses (T1562 - 001)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "dll_block"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Scheduled Transfer (T1029)"
            }
          ],
          "name": "Exfiltration (TA0010)"
        }
      ],
      "name": "sleep"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "arp"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Network Share Discovery (T1135)"
            },
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "sharescan"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "phantom_thread"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Proxy (T1090)"
            }
          ],
          "name": "Command and Control (TA0011)"
        }
      ],
      "name": "rportfwd"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        },
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (1059 - 001)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "psreflect"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 002)"
            },
            {
              "name": "System Services: Service Execution (T1569 - 002)"
            }
          ],
          "name": "Execution (TA0002)"
        },
        {
          "children": [
            {
              "name": "Create or Modify System Process: Windows Service (T1543 - 003)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "psexec"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Screen Capture (T1113)"
            }
          ],
          "name": "Collection (TA0009)"
        }
      ],
      "name": "screenshot"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Discovery (T1057)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "psgrep"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (T1059 - 001, 003)"
            }
          ],
          "name": "Execution (TA0002)"
        },
        {
          "children": [
            {
              "name": "Abuse Elevation Control Mechanism (T1548 - 002)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "shellspawn"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Create or Modify System Process: Windows Service (T1543 - 003)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "scdivert"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "lsdr"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Information Discovery (T1082)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "uptime"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Ingress Tool Transfer (T1105)"
            }
          ],
          "name": "Command and Control (TA0011)"
        },
        {
          "children": [
            {
              "name": "Lateral Tool Transfer (T1570)"
            }
          ],
          "name": "Lateral Movement (TA0008)"
        }
      ],
      "name": "cp"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        },
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (1059 - 001)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "sharpreflect"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "routes"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Data Destruction (T1485)"
            }
          ],
          "name": "Impact (TA0040)"
        }
      ],
      "name": "rmdir"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Fallback Channels (T1008)"
            }
          ],
          "name": "Command and Control (TA0011)"
        }
      ],
      "name": "switch_profile"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Windows Management Instrumentation (T1047)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "wmiexec"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Application Layer Protocol (T1071 - 001)"
            },
            {
              "name": "Encrypted Channel (T1573)"
            }
          ],
          "name": "Command and Control (TA0011)"
        }
      ],
      "name": "HTTP Badger"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Access Token Manipulation (T1134 - 001)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "revtoken"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Owner/User Discovery (T1033)"
            },
            {
              "name": "Permission Groups Discovery (T1069 - 001, 002)"
            },
            {
              "name": "Account Discovery (T1087 - 001, 002)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "net"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "dnscache"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001, 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "memexec"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Discovery (T1057)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "ps"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Application Layer Protocol (T1071 - 001)"
            },
            {
              "name": "Data Encoding (T1132 - 002)"
            },
            {
              "name": "Data Obfuscation (T1001 - 001)"
            },
            {
              "name": "Encrypted Channel (T1573)"
            }
          ],
          "name": "Command and Control (TA0011)"
        }
      ],
      "name": "upload"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Masquerading (T1036 - 004)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "set_argument"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Services: Service Execution (T1569 - 002)"
            }
          ],
          "name": "Execution (TA0002)"
        },
        {
          "children": [
            {
              "name": "Create or Modify System Process: Windows Service (T1543 - 003)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "sccreate"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "OS Credential Dumping (T1003.001)"
            }
          ],
          "name": "Credential Access (TA0006)"
        }
      ],
      "name": "shadowcloak"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Scheduled Task (T1053 - 005)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "schtquery"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Screen Capture (T1113)"
            }
          ],
          "name": "Collection (TA0009)"
        }
      ],
      "name": "record_screen"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "local_sessions"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Valid Accounts (T1078 - 002, 003)"
            },
            {
              "name": "Access Token Manipulation (T1134 - 001, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "make_token"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Valid Accounts (T1078 - 002, 003)"
            },
            {
              "name": "Access Token Manipulation (T1134 - 001, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "impersonate"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Discovery (T1057)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "list_exports"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Account Discovery (T1087 - 001, 002)"
            },
            {
              "name": "Domain Trust Discovery (T1482)"
            },
            {
              "name": "File and Directory Discovery (T1083)"
            },
            {
              "name": "Permission Groups Discovery (T1069 - 001, 002)"
            },
            {
              "name": "Remote System Discovery (T1018)"
            },
            {
              "name": "System Information Discovery (T1082)"
            },
            {
              "name": "System Owner/User Discovery (T1033)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "sentinel"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Data Destruction (T1485)"
            }
          ],
          "name": "Impact (TA0040)"
        }
      ],
      "name": "rm"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Impair Defenses (T1562 - 001)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "kill"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Input Capture (T1056 - 002)"
            }
          ],
          "name": "Credential Access (TA0006)"
        }
      ],
      "name": "phish_creds"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "preview"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Data Encoding (T1132 - 002)"
            },
            {
              "name": "Encrypted Channel (T1573)"
            },
            {
              "name": "Non-Standard Port (T1571)"
            }
          ],
          "name": "Command and Control (TA0011)"
        },
        {
          "children": [
            {
              "name": "Lateral Tool Transfer (T1570)"
            },
            {
              "name": "Remote Services (T1021 - 002)"
            }
          ],
          "name": "Lateral Movement (TA0008)"
        }
      ],
      "name": "pivot_tcp"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Input Capture: Keylogging (T1056 - 001)"
            }
          ],
          "name": "Collection (TA0009)"
        },
        {
          "children": [
            {
              "name": "Input Capture: Web Portal Capture (T1056 - 002, 003)"
            }
          ],
          "name": "Credential Access (TA0006)"
        }
      ],
      "name": "keylogger"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Software Discovery (T1418)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "applist"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "pwd"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Clipboard Data (T1115)"
            }
          ],
          "name": "Collection (TA0009)"
        }
      ],
      "name": "dumpclip"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Software Discovery (T1518)"
            },
            {
              "name": "System Service Discovery (T1007)"
            }
          ],
          "name": "Collection (TA0009)"
        }
      ],
      "name": "drivers"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Service Discovery (T1007)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "scquery"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Network Share Discovery (T1135)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "netshares"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "netstat"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Valid Accounts (T1078 - 002, 003)"
            },
            {
              "name": "Access Token Manipulation (T1134 - 001, 002, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "runas"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "shinject_ex"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "OS Credential Dumping (T1003.001)"
            }
          ],
          "name": "Credential Access (TA0006)"
        }
      ],
      "name": "memdump"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Access Token Manipulation (T1134 - 001, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "addpriv"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Service Stop (T1489)"
            }
          ],
          "name": "Impact (TA0040)"
        }
      ],
      "name": "scstop"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Network Service Scanning (T1046)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "portscan"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Ingress Tool Transfer (T1105)"
            }
          ],
          "name": "Command and Control (TA0011)"
        }
      ],
      "name": "curl"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Use Alternate Authentication Material: Pass the Hash (T1550 - 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        },
        {
          "children": [
            {
              "name": "Valid Accounts (T1078 - 002, 003)"
            },
            {
              "name": "Access Token Manipulation (T1134 - 001, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        },
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (T1059 - 001, 003)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "pth"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Proxy (T1090 - 001, 002)"
            },
            {
              "name": "Remote Access Software (T1219)"
            },
            {
              "name": "Protocol Tunneling (T1572)"
            },
            {
              "name": "Non-Application Layer Protocol (T1095)"
            }
          ],
          "name": "Command and Control (TA0011)"
        }
      ],
      "name": "socks"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Application Layer Protocol (T1071 - 001)"
            },
            {
              "name": "Data Encoding (T1132 - 002)"
            },
            {
              "name": "Data Obfuscation (T1001 - 001)"
            },
            {
              "name": "Encrypted Channel (T1573)"
            }
          ],
          "name": "Command and Control (TA0011)"
        },
        {
          "children": [
            {
              "name": "Data Transfer Size Limits (T1030)"
            },
            {
              "name": "Exfiltration Over C2 Channel (T1041)"
            }
          ],
          "name": "Exfiltration (TA0010)"
        }
      ],
      "name": "download"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "OS Credential Dumping (T1003 - 006)"
            }
          ],
          "name": "Credential Access (TA0006)"
        }
      ],
      "name": "dcsync"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001, 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        },
        {
          "children": [
            {
              "name": "OS Credential Dumping (T1003 - 006)"
            }
          ],
          "name": "Credential Access (TA0006)"
        }
      ],
      "name": "mimikatz"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Services: Service Execution (T1569 - 002)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "scstart"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Information Discovery (T1082)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "sysinfo"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Endpoint Denial of Service (T1499)"
            }
          ],
          "name": "Impact (TA0040)"
        }
      ],
      "name": "lockws"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (T1059 - 001, 003)"
            }
          ],
          "name": "Execution (TA0002)"
        },
        {
          "children": [
            {
              "name": "Access Token Manipulation (T1134 - 001, 002, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "system_exec"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001, 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "loadr"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Windows Management Instrumentation (T1047)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "wmiquery"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Steal or Forge Kerberos Tickets: Kerberoasting (T1558 - 003)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "kerberoast"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Information Discovery (T1082)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "idletime"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "query_session"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Discovery (T1057)"
            },
            {
              "name": "Remote System Discovery (T1018)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "ps_ex"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Permission Groups Discovery (T1069 - 001, 002)"
            },
            {
              "name": "System Owner/User Discovery (T1033)"
            },
            {
              "name": "Account Discovery (T1087 - 001, 002)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "userinfo"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Ingress Tool Transfer (T1105)"
            }
          ],
          "name": "Command and Control (TA0011)"
        },
        {
          "children": [
            {
              "name": "Lateral Tool Transfer (T1570)"
            }
          ],
          "name": "Lateral Movement (TA0008)"
        }
      ],
      "name": "mv"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (1059 - 001)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "sharpinline"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Remote System Discovery (T1018)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "icmp_ping"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Password Policy Discovery (T1201)"
            },
            {
              "name": "Remote System Discovery (T1018)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "dcenum"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        },
        {
          "children": [
            {
              "name": "Remote Services (T1021 - 002)"
            }
          ],
          "name": "Lateral Movement (TA0008)"
        }
      ],
      "name": "cd"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (T1059 - 001, 003)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "run"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Hooking (NA)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "detect_hooks"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Access Token Manipulation (T1134 - 004)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "set_parent"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Query Registry (T1012)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "reg"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Discovery (T1057)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "list_modules"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Command and Scripting Interpreter (T1059 - 001, 003)"
            }
          ],
          "name": "Execution (TA0002)"
        }
      ],
      "name": "suspended_run"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001)"
            },
            {
              "name": "Deobfuscate/Decode Files or Information (T1140)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        },
        {
          "children": [
            {
              "name": "Data Encrypted for Impact (T1486)"
            }
          ],
          "name": "Impact (TA0040)"
        }
      ],
      "name": "cryptvortex"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Access Token Manipulation (T1134 - 001, 003)"
            }
          ],
          "name": "Privilege Escalation (TA0004)"
        }
      ],
      "name": "get_system"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Password Policy Discovery (T1201)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "passpol"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "fileinfo"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "System Network Configuration Discovery (T1016)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "ipstats"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Data Staged (T1074 - 001)"
            }
          ],
          "name": "Collection (TA0009)"
        }
      ],
      "name": "mkdir"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Network Share Discovery (T1135)"
            },
            {
              "name": "File and Directory Discovery (T1083)"
            }
          ],
          "name": "Discovery (TA0007)"
        }
      ],
      "name": "ls"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001, 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        }
      ],
      "name": "set_child"
    },
    {
      "children": [
        {
          "children": [
            {
              "name": "Process Injection (T1055 - 001, 002)"
            }
          ],
          "name": "Defense Evasion (TA0005)"
        },
        {
          "children": [
            {
              "name": "OS Credential Dumping (T1003 - 002)"
            }
          ],
          "name": "Credential Access (TA0006)"
        }
      ],
      "name": "samdump"
    }
  ],
  "name": "Brute Ratel MITRE Map"
}"#;
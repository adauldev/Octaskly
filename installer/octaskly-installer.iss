; Octaskly Installer - Windows (Inno Setup)
; Installer profesional untuk Windows
; Versi: 1.0.0
; Lisensi: MIT

#define MyAppName "Octaskly"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Octaskly"
#define MyAppURL "https://github.com/adauldev/octaskly"
#define MyAppExeName "octaskly.exe"

[Setup]
AppId={{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}/releases
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=..\LICENSE
OutputDir=..\dist\windows
OutputBaseFilename=Octaskly Installer
SetupIconFile=..\img\logo\-icon-octaskly.ico
Compression=lz4
SolidCompression=yes
PrivilegesRequired=admin
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: "indonesian"; MessagesFile: "compiler:Languages\Indonesian.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "addtopath"; Description: "Tambahkan Octaskly ke PATH sistem"; GroupDescription: "Integrasi Sistem"; Flags: checkedonce

[Files]
; Copy the built binary
Source: "..\target\release\octaskly.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--help"; AppUserModelID: "OctaskLy.CLI"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--help"; Tasks: desktopicon; AppUserModelID: "OctaskLy.CLI"

[Run]
; Tampilkan versi setelah instalasi
Filename: "{cmd}"; Parameters: "/c {app}\{#MyAppExeName} --version"; Description: "Tampilkan versi"; Flags: runhidden

[Code]
procedure RegisterPathEntry;
var
  OldPath: string;
  NewPath: string;
  InstallPath: string;
begin
  if not IsTaskSelected('addtopath') then
    Exit;

  InstallPath := ExpandConstant('{app}');
  
  if RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OldPath) then
  begin
    if Pos(InstallPath, OldPath) = 0 then
    begin
      NewPath := OldPath;
      if Length(NewPath) > 0 then
        NewPath := NewPath + ';';
      NewPath := NewPath + InstallPath;
      
      RegWriteStringValue(HKEY_LOCAL_MACHINE,
        'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
        'Path', NewPath);
    end;
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    RegisterPathEntry;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  OldPath: string;
  NewPath: string;
  InstallPath: string;
  PathIndex: integer;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    InstallPath := ExpandConstant('{app}');
    
    if RegQueryStringValue(HKEY_LOCAL_MACHINE,
      'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
      'Path', OldPath) then
    begin
      NewPath := OldPath;
      PathIndex := Pos(InstallPath, NewPath);
      
      if PathIndex > 0 then
      begin
        if NewPath[PathIndex - 1] = ';' then
          Dec(PathIndex);
        
        Delete(NewPath, PathIndex, Length(InstallPath));
        
        if Length(NewPath) > 0 then
        begin
          if NewPath[Length(NewPath)] = ';' then
            SetLength(NewPath, Length(NewPath) - 1);
        end;
        
        RegWriteStringValue(HKEY_LOCAL_MACHINE,
          'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
          'Path', NewPath);
      end;
    end;
  end;
end;

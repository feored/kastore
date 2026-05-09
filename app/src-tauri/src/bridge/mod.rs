use std::path::Path;

use kastore::{Diagnostic, SaveGame, SaveString, ValidationIssue};

use crate::dto::{
    DiagnosticDto, OpenedSaveDto, SaveStringDto, ScenarioDto, SourceDto, ValidationIssueDto,
    ValidationResultDto,
};

pub fn opened_save_dto(
    path: &Path,
    save: &SaveGame,
    diagnostics: Vec<DiagnosticDto>,
    dirty: bool,
    revision: u64,
) -> OpenedSaveDto {
    OpenedSaveDto {
        source: source_dto(path, save),
        scenario: scenario_dto(save),
        diagnostics,
        dirty,
        revision,
    }
}

pub fn scenario_dto(save: &SaveGame) -> ScenarioDto {
    let file_info = &save.header.file_info;

    ScenarioDto {
        name: save_string_dto(&file_info.name),
        file_name: save_string_dto(&file_info.filename),
        description: save_string_dto(&file_info.description),
        width: file_info.width,
        height: file_info.height,
        difficulty: file_info.difficulty.to_string(),
        language: file_info.main_language.to_string(),
        game_type: save.header.game_type.to_string(),
        requires_pol: save.header.requires_pol,
    }
}

pub fn parser_diagnostics(diagnostics: &[Diagnostic]) -> Vec<DiagnosticDto> {
    diagnostics.iter().map(diagnostic_dto).collect()
}

pub fn validation_result(revision: u64, issue: Option<ValidationIssue>) -> ValidationResultDto {
    ValidationResultDto {
        revision,
        issues: issue.into_iter().map(validation_issue_dto).collect(),
    }
}

fn source_dto(path: &Path, save: &SaveGame) -> SourceDto {
    SourceDto {
        path: path.to_string_lossy().into_owned(),
        file_name: path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default(),
        save_version: save.source_version.as_u16(),
    }
}

fn save_string_dto(value: &SaveString) -> SaveStringDto {
    SaveStringDto {
        text: value.to_string_lossy(),
        raw_bytes: value.as_bytes().to_vec(),
        valid_utf8: value.as_utf8().is_ok(),
        modified: false,
    }
}

fn diagnostic_dto(diagnostic: &Diagnostic) -> DiagnosticDto {
    DiagnosticDto {
        severity: diagnostic.severity.to_string(),
        kind: diagnostic.kind.to_string(),
        section: diagnostic.section.to_string(),
        field: diagnostic.field.map(str::to_string),
        offset: diagnostic.offset,
        message: diagnostic.message.clone(),
    }
}

fn validation_issue_dto(issue: ValidationIssue) -> ValidationIssueDto {
    ValidationIssueDto {
        field: issue.field.to_string(),
        message: issue.message.to_string(),
    }
}

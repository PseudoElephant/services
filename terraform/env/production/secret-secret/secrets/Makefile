SHELL := /bin/bash

TF_VAR_service_name := terraform-configuration
export TF_VAR_service_name

terraform_init = terraform init
terraform_apply = terraform apply -var-file=../../../../tfvars/$(1).tfvars
terraform_plan = terraform apply -var-file=../../../../tfvars/$(1).tfvars
terraform_destroy = terraform destroy -var-file=../../../../tfvars/$(1).tfvars


check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))

.PHONY: all test clean

plan:
	@:$(call check_defined, stage, stage name)
	echo "Creating Plan ${TF_VAR_service_name} to ${stage}" \
		&& $(call terraform_init) \
		&& $(call terraform_plan,$(strip ${stage}),$(strip ${terraform_args}))

deploy:
	@:$(call check_defined, stage, stage name)
	echo "Deploying ${TF_VAR_service_name} to ${stage}" \
		&& $(call terraform_init) \
		&& $(call terraform_apply,$(strip ${stage}),$(strip ${terraform_args}))

teardown:
	@:$(call check_defined, stage, stage name)
	echo "Destroying ${TF_VAR_service_name}, ${stage}" \
		&& $(call terraform_init) \
		&& $(call terraform_destroy,$(strip ${stage}),$(strip ${terraform_args}))